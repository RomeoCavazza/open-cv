const $ = (s) => document.querySelector(s);
const alertBox = $('#alert');
const tbody = $('#resultTable tbody');
const btn = $('#btnSearch');

function showError(text) {
  alertBox.className = 'alert alert-danger';
  alertBox.textContent = text;
  alertBox.classList.remove('d-none');
}
function clearError() {
  alertBox.classList.add('d-none');
  alertBox.textContent = '';
}
function clearTable() {
  tbody.innerHTML = '';
}

btn.addEventListener('click', () => {
  clearError();
  clearTable();

  const params = new URLSearchParams({
    type:   $('#type').value.trim(),
    brand:  $('#brand').value.trim(),
    price:  $('#price').value.trim(),
    number: $('#number').value.trim(),
  });

  fetch(`task05.php?${params.toString()}`, { method: 'GET' })
    .then((r) => {
      if (!r.ok) throw new Error('HTTP error ' + r.status);
      return r.json();
    })
    .then((data) => {
      if (data.error) {
        showError(data.error);
        return;
      }

      if (!Array.isArray(data.products) || data.products.length === 0) {
        showError('No products found.');
        return;
      }
      const rows = data.products.map(p => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
          <td>${p.type}</td>
          <td>${p.brand}</td>
          <td>${p.price}</td>
          <td>${p.number}</td>
          <td>${p.stock}</td>
        `;
        return tr;
      });
      rows.forEach(tr => tbody.appendChild(tr));
    })
    .catch(() => {
      showError('An error occurred while contacting the server.');
    });
});
