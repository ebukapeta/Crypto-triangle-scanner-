const tableWrap = document.getElementById('table-wrap');
const statusEl = document.getElementById('status');

function buildTable(rows) {
  const table = document.createElement('table');
  table.innerHTML = `
    <thead>
      <tr>
        <th>Triangle Pair</th>
        <th>Profit % (Before Fees)</th>
        <th>Trade Fees (%)</th>
        <th>Profit % (After Fees)</th>
      </tr>
    </thead>
    <tbody></tbody>
  `;
  const tbody = table.querySelector('tbody');
  rows.forEach(r => {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td>${r.triangle}</td>
      <td>${r.profit_before_fees.toFixed(2)}%</td>
      <td>${r.trade_fees.toFixed(2)}%</td>
      <td>${r.profit_after_fees.toFixed(2)}%</td>
    `;
    tbody.appendChild(tr);
  });
  return table;
}

async function scan(exchange) {
  const minProfit = Number(document.getElementById('minProfit').value || 0.3);
  const feePerc   = Number(document.getElementById('feePerc').value || 0.1);

  statusEl.textContent = `Scanning ${exchange}…`;
  tableWrap.innerHTML = '';

  try {
    const url = `/${exchange}/triangular?min_profit=${minProfit}&fee_perc=${feePerc}`;
    const res = await fetch(url);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    statusEl.textContent = `Found ${data.length} opportunities on ${exchange}.`;
    tableWrap.appendChild(buildTable(data));
  } catch (e) {
    statusEl.textContent = `Error: ${e.message}`;
  }
}

document.querySelectorAll('button[data-ex]').forEach(btn => {
  btn.addEventListener('click', () => scan(btn.getAttribute('data-ex')));
});
