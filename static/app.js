document.getElementById("scanBtn").addEventListener("click", async () => {
  const exchange = document.getElementById("exchange").value;
  const minProfit = parseFloat(document.getElementById("minProfit").value) || 0.3;
  const tableBody = document.querySelector("#resultsTable tbody");

  tableBody.innerHTML = `<tr><td colspan="5" class="placeholder">Scanning ${exchange}...</td></tr>`;

  try {
    const res = await fetch(`/api/${exchange}/triangular?min_profit=${minProfit}`);
    const data = await res.json();

    if (!data || data.length === 0) {
      tableBody.innerHTML = `<tr><td colspan="5" class="placeholder">No opportunities found</td></tr>`;
      return;
    }

    tableBody.innerHTML = "";
    data.forEach((row) => {
      const pairs = row.triangle
        .split(" -> ")
        .map((seg) => seg.replace("/", "-"))
        .join(" → ");

      const tr = document.createElement("tr");
      tr.innerHTML = `
        <td>${row.triangle}</td>
        <td class="text-gray">${pairs}</td>
        <td class="green">${row.profit_before_fees.toFixed(2)}%</td>
        <td class="yellow">${row.trade_fees.toFixed(2)}%</td>
        <td class="${row.profit_after_fees > 0 ? "green" : "red"}">
          ${row.profit_after_fees.toFixed(2)}%
        </td>
      `;
      tableBody.appendChild(tr);
    });
  } catch (err) {
    console.error(err);
    tableBody.innerHTML = `<tr><td colspan="5" class="placeholder">Error fetching data</td></tr>`;
  }
});
