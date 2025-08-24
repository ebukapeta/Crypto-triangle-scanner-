const exchanges = ["binance", "bybit", "kucoin", "gateio", "kraken"];

async function fetchData(exchange) {
  const response = await fetch(`/${exchange}/triangular`);
  return response.json();
}

async function renderTables() {
  const container = document.getElementById("tables");
  container.innerHTML = "";

  for (const exchange of exchanges) {
    const data = await fetchData(exchange);

    const table = document.createElement("table");
    const caption = document.createElement("caption");
    caption.textContent = `${exchange.toUpperCase()} Opportunities`;
    table.appendChild(caption);

    table.innerHTML += `
      <tr>
        <th>Triangle Path</th>
        <th>Profit % Before Fees</th>
        <th>Trade Fees (%)</th>
        <th>Profit % After Fees</th>
      </tr>
    `;

    data.forEach(item => {
      table.innerHTML += `
        <tr>
          <td>${item.triangle}</td>
          <td>${item.profit_before_fees.toFixed(2)}%</td>
          <td>${item.trade_fees.toFixed(2)}%</td>
          <td>${item.profit_after_fees.toFixed(2)}%</td>
        </tr>
      `;
    });

    container.appendChild(table);
  }
}

setInterval(renderTables, 5000); // Refresh every 5 seconds
renderTables();
