const form = document.getElementById('taskForm');
const input = document.getElementById('taskInput');
const select = document.getElementById('taskType');
const list = document.getElementById('taskList');
const searchForm = document.getElementById('searchForm');
const searchType = document.getElementById('searchType');
const resetBtn = document.getElementById('resetBtn');

form.addEventListener('submit', (e) => {
  e.preventDefault();
  const li = document.createElement('li');
  li.textContent = input.value;
  li.classList.add(select.value);
  list.appendChild(li);
  input.value = '';
});

searchForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const type = searchType.value;
  document.querySelectorAll('#taskList li').forEach(li => {
    li.style.display = li.classList.contains(type) ? '' : 'none';
  });
});

resetBtn.addEventListener('click', () => {
  document.querySelectorAll('#taskList li').forEach(li => {
    li.style.display = '';
  });
});


