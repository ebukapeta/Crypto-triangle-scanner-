const scanBtn = document.getElementById('scanBtn');
const exchangeEl = document.getElementById('exchange');
const minProfitEl = document.getElementById('minProfit');
const statusEl = document.getElementById('status');
const resultsEl = document.getElementById('results');

scanBtn.addEventListener('click', () => scanNow());

async function scanNow() {
  resultsEl.innerHTML = '';
  const exchange = exchangeEl.value;
  const minProfit = parseFloat(minProfitEl.value) || 0.3;
  statusEl.textContent = 'Scanning...';

  const url = `/api/${exchange}/triangular?min_profit=${encodeURIComponent(minProfit)}`;

  try {
    const res = await fetch(url);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();

    if (!Array.isArray(data) || data.length === 0) {
      statusEl.textContent = 'No opportunities found (or exchange API returned no valid data).';
      return;
    }

    // Build table
    const tbl = document.createElement('table');
    tbl.innerHTML = `
      <thead>
        <tr>
          <th>Triangle</th>
          <th>Profit % (before fees)</th>
          <th>Trade Fees % (total)</th>
          <th>Profit % (after fees)</th>
        </tr>
      </thead>
      <tbody></tbody>
    `;
    const tbody = tbl.querySelector('tbody');

    data.forEach(r => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td>${r.triangle}</td>
        <td>${(Number(r.profit_before_fees)).toFixed(4)}%</td>
        <td>${(Number(r.trade_fees)).toFixed(4)}%</td>
        <td>${(Number(r.profit_after_fees)).toFixed(4)}%</td>
      `;
      tbody.appendChild(tr);
    });

    resultsEl.appendChild(tbl);
    statusEl.textContent = `Done. Found ${data.length} triangle(s).`;

  } catch (err) {
    console.error(err);
    statusEl.textContent = `Error fetching data: ${err.message}`;
    resultsEl.innerHTML = `<p>Error fetching data: ${err.message}</p>`;
  }
              }
