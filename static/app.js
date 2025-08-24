const scanBtn = document.getElementById('scanBtn');
const exchangeEl = document.getElementById('exchange');
const minProfitEl = document.getElementById('minProfit');
const feeEl = document.getElementById('fee');
const statusEl = document.getElementById('status');
const resultsEl = document.getElementById('results');

scanBtn.addEventListener('click', () => {
  scanNow();
});

async function scanNow() {
  const exchange = exchangeEl.value;
  const minProfit = parseFloat(minProfitEl.value) || 0.3;
  const fee = parseFloat(feeEl.value) || 0.1;

  statusEl.textContent = `Scanning ${exchange} ...`;
  resultsEl.innerHTML = '';

  try {
    const res = await fetch(`/${exchange}/triangular?min_profit=${minProfit}&fee_perc=${fee}`);
    if (!res.ok) throw new Error('Network error');
    const data = await res.json();

    statusEl.textContent = `Found ${data.length} opportunities`;

    if (data.length === 0) {
      resultsEl.innerHTML = '<p>No opportunities found.</p>';
      return;
    }

    const tbl = document.createElement('table');
    tbl.innerHTML = `
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
    const tbody = tbl.querySelector('tbody');

    data.forEach(r => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td>${r.triangle}</td>
        <td>${Number(r.profit_before_fees).toFixed(2)}%</td>
        <td>${Number(r.trade_fees).toFixed(2)}%</td>
        <td>${Number(r.profit_after_fees).toFixed(2)}%</td>
      `;
      tbody.appendChild(tr);
    });

    resultsEl.appendChild(tbl);

  } catch (err) {
    console.error(err);
    statusEl.textContent = `Error: ${err.message}`;
    resultsEl.innerHTML = `<p>Error fetching data: ${err.message}</p>`;
  }
      }
