import { useRef, useEffect, useReducer, useCallback } from "react";
import Table from "./components/Table";
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from "@tauri-apps/api/core";
import { webview } from "@tauri-apps/api";
import "./App.css";
import logo from "./assets/convertsimp_logo.svg"
// import { checkFullDiskAccessPermissions, requestFullDiskAccessPermissions } from "tauri-plugin-macos-permissions-api";

function getThisWebview() {
  return webview.getCurrentWebview()
}

const EMOJIS = {
  success: '✅',
  error: '❌',
  undefined: '🕙'
}

const INITIAL_STATE = {
  paths: [],
  statusMap: {},
  loading: false,
}

function App() {
  const inputFile = useRef()
  const unlistenHandlers = useRef([])

  const [state, setState] = useReducer(
    (st, a) => a === 'reset'
      ? INITIAL_STATE
      : ({
        ...st,
        paths: [...st.paths, ...(a.paths || [])],
        statusMap: {...st.statusMap, ...(a.statusMap || {})},
        loading: a.loading || st.loading,
      }),
    INITIAL_STATE
  )

  const [statusMap, setStatus] = useReducer((st, a) => {
    if (a === 'reset') return {}
    return {
      ...st,
      [a.path]: a.success ? 'success' : 'error'
    }
  }, {})

  const convert = useCallback(async () => {
    const toProcess = state.paths.filter(path => !state.statusMap[path])
    await invoke("use_ffmpeg", { paths: state.paths })
  }, [state.paths, state.statusMap])

  const selectFile = useCallback(async () => {
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'lossless',
        extensions: ['wav', 'aif']
      }]
    });

    if (Array.isArray(selected)) {
      setState({ paths: selected })
    }
  }, [])

  const onInputClick = useCallback((e) => {
    e.preventDefault()
    selectFile()
  }, [selectFile])

    const downloadAndInstallFFMPEG = useCallback(async () => {
      invoke('download_install_ffmpeg')

      setState({ loading: true })
    })

  useEffect(() => {
    const subscribe = async () => {
      unlistenHandlers.current.push(getThisWebview().listen('tauri://drag-drop', (e) => {
        const selected = e.payload?.paths
        if (Array.isArray(selected)) {
          setState({ paths: selected })
        }
      }))

      unlistenHandlers.current.push(getThisWebview().listen('path-processed', (e) => {
        const { path, success, error } = e.payload
        setState({
          statusMap: {
            [path]: success ? 'success' : 'error'
          }
        })
      }))

      unlistenHandlers.current.push(getThisWebview().listen('ffmpeg-download-finished', (e) => {
        setState({ loading: false })
      }))
    }


    const getFFMPEGVerson = async () => {
      const version = await invoke('get_ffmpeg_version')
      setState({ version })
    }

    subscribe()

    // checkFullDiskAccessPermissions().then(authorized => {
      // console.log(authorized); // true
      // if (!authorized) {
        // return requestFullDiskAccessPermissions()
      // }
    // });

    invoke('init_appdatadir')

    getFFMPEGVerson()

    // installFFMPEG()

    return () => {
      for (const unlisten of unlistenHandlers.current) {
        if (typeof unlisten === 'function') {
          unlisten()
        }
      }
      unlistenHandlers.current = []
    }
  }, [])

  return (
    <main className="flex column h-full w-full">
      <div className="header flex w-full relative">
        <img src={logo} className="logo" />
        <div style={{ alignItems: 'flex-start' }}>
          {state.paths.length > 0
            ? (
              <button
                style={{ background: "#4c4c4c" }}
                onClick={() => {
                  setStatus('reset')
                }}
              >
                Start Over
              </button>
            ) : (<div></div>)}
        </div>
        <div className="button-container flex end grow">
          <button
            style={{ background: "#4c4c4c" }}
            onClick={() => {
            }}
          >
            ⚙ Settings
          </button>
          <button disabled={state.paths.length === 0} onClick={convert}>
            ↯ Convert to &nbsp;
            <select id="outputext" onClick={e => e.stopPropagation()}>
              <option>mp3</option>
              <option>wav</option>
              <option>aif</option>
              <option>aif</option>
            </select>
          </button>
        </div>
      </div>
      <label htmlFor="inputFile" />
      <input ref={inputFile} type="file" id="inputFile" onClick={onInputClick} />
      <div className="flex" style={{ height: 3 }} />
      <div
        className="flex center grow"
        style={{
          margin: '5px 7px',
          overflow: 'hidden',
          ...(state.paths.length ? { background: 'rgba(255, 255, 255, .013)' } : {})
        }}
      >
        {state.paths.length
          ? (
            <Table
              rows={[
                ["File","Status"],
                ...(state.paths.map(filename => [filename, EMOJIS[statusMap[filename]]]))
              ]}
            />
          )
          : (
            <div
              className="dropbox flex w-full stretch center"
              onClick={() => inputFile.current.click()}
            >
              drop files here or click to select
            </div>
        )}
      </div>
    </main>
  );
}

export default App;
