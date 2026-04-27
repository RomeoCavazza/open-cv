const input = document.getElementById('listItem');
const button = document.querySelector('button');

button.addEventListener('click', () => {
  const div = document.createElement('div');
  div.textContent = input.value;
  input.insertAdjacentElement('afterend', div);
});


