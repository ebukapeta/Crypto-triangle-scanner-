const form = document.getElementById('scan-form');
const exchangeSel = document.getElementById('exchange');
const minProfit = document.getElementById('min-profit');
const statusEl = document.getElementById('status');
const tbody = document.querySelector('#results tbody');

form.addEventListener('submit', async (e) => {
  e.preventDefault();
  tbody.innerHTML = '';
  statusEl.textContent = 'Scanning...';

  const ex = exchangeSel.value;
  const min = parseFloat(minProfit.value || '0.3');

  try {
    const url = `/api/${ex}/triangular?min_profit=${encodeURIComponent(min)}`;
    const res = await fetch(url);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();

    if (!Array.isArray(data) || data.length === 0) {
      statusEl.textContent = 'No opportunities found (or the exchange API limit was hit).';
      return;
    }

    for (const row of data) {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td>${row.triangle}</td>
        <td>${fmt(row.profit_before_fees)}</td>
        <td>${fmt(row.trade_fees)}</td>
        <td>${fmt(row.profit_after_fees)}</td>
      `;
      tbody.appendChild(tr);
    }
    statusEl.textContent = `Done. Showing ${data.length} triangle(s).`;
  } catch (err) {
    statusEl.textContent = `Error fetching data: ${err.message}`;
  }
});

function fmt(v) {
  return (typeof v === 'number' && isFinite(v)) ? v.toFixed(2) : '-';
}
