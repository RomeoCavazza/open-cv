const form = document.getElementById('taskForm');
const input = document.getElementById('taskInput');
const select = document.getElementById('taskType');
const list = document.getElementById('taskList');
const searchForm = document.getElementById('searchForm');
const searchText = document.getElementById('searchText');
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
  const text = searchText.value.toLowerCase();
  const type = searchType.value;
  
  document.querySelectorAll('#taskList li').forEach(li => {
    const matchesText = !text || li.textContent.toLowerCase().includes(text);
    const matchesType = !type || li.classList.contains(type);
    li.style.display = matchesText && matchesType ? '' : 'none';
  });
});

resetBtn.addEventListener('click', () => {
  searchText.value = '';
  searchType.value = '';
  document.querySelectorAll('#taskList li').forEach(li => {
    li.style.display = '';
  });
});


