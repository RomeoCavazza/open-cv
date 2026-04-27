const form = document.getElementById('taskForm');
const input = document.getElementById('taskInput');
const select = document.getElementById('taskType');
const list = document.getElementById('taskList');

form.addEventListener('submit', (e) => {
  e.preventDefault();
  const li = document.createElement('li');
  li.textContent = input.value;
  li.classList.add(select.value);
  list.appendChild(li);
  input.value = '';
});


