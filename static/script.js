async function scan() {
  const exchange = document.getElementById('exchange').value;
  const profit = document.getElementById('profit').value;

  const response = await fetch(`/scan?exchange=${exchange}&profit=${profit}`);
  const data = await response.json();

  document.getElementById('results').innerText = JSON.stringify(data, null, 2);
}
