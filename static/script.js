async function startScan() {
  const exchange = document.getElementById('exchange').value;
  const profit = parseFloat(document.getElementById('profit').value);

  const resp = await fetch(`/opportunities?exchange=${exchange}&min_profit=${profit}`);
  const data = await resp.json();

  const tbody = document.getElementById('results').querySelector('tbody');
  tbody.innerHTML = '';

  data.forEach(op => {
    const tr = document.createElement('tr');
    tr.innerHTML = `<td>${op.path.join(' → ')}</td><td>${op.profit_pct.toFixed(3)}%</td><td>${new Date().toLocaleTimeString()}</td>`;
    tbody.appendChild(tr);
  });
}