function Table({ rows }) {
  return (
    <table>
      <thead>
        {rows[0].map((label, i) => (
          <td key={i}>
            {label}
          </td>
        ))}
      </thead>
      <tbody>
        {rows.slice(1).map((row, i) => (
          <tr key={i}>
            {row.map(col => (
              <td>
                {col}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  )
}

export default Table;
