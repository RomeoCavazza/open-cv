const typeInput  = document.getElementById('type');
const brandInput = document.getElementById('brand');

const typeMsg  = document.getElementById('typeMsg');
const brandMsg = document.getElementById('brandMsg');
const alertBox = document.getElementById('alert');

const setMsg = (el, text, ok) => {
  el.textContent = text;
  el.classList.remove('text-success','text-danger');
  el.classList.add(ok ? 'text-success' : 'text-danger');
};

const localValidate = () => {
  const type  = (typeInput.value || '').trim();
  const brand = (brandInput.value || '').trim();

  const params = new URLSearchParams({
    type, brand
  });

  fetch(`task04.php?${params.toString()}`, { method: 'GET' })
    .then(r => r.json())
    .then(data => {
      setMsg(typeMsg,  data.type.message,  data.type.ok);
      setMsg(brandMsg, data.brand.message, data.brand.ok);
      alertBox.classList.add('d-none');
    })
    .catch(() => {
      alertBox.className = 'alert alert-danger';
      alertBox.textContent = 'An error occurred while contacting the server.';
      alertBox.classList.remove('d-none');
    });
};

[typeInput, brandInput].forEach(i => {
  i.addEventListener('input', localValidate);
});

localValidate();