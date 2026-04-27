const form = document.getElementById('taskForm');
const input = document.getElementById('taskInput');
const select = document.getElementById('taskType');
const list = document.getElementById('taskList');
const searchForm = document.getElementById('searchForm');
const searchText = document.getElementById('searchText');
const searchType = document.getElementById('searchType');
const searchTags = document.getElementById('searchTags');
const resetBtn = document.getElementById('resetBtn');

form.addEventListener('submit', (e) => {
  e.preventDefault();
  const li = document.createElement('li');
  li.classList.add(select.value);
  li.dataset.tags = '';
  
  const content = document.createElement('div');
  content.className = 'task-item';
  content.innerHTML = `
    <span class="task-content">${input.value}</span>
    <span class="task-tags"></span>
    <button class="add-tag-btn">+ Tag</button>
  `;
  
  li.appendChild(content);
  list.appendChild(li);
  input.value = '';
  
  content.querySelector('.add-tag-btn').addEventListener('click', () => addTag(li));
});

function addTag(li) {
  const tag = prompt('Enter tag:');
  if (tag) {
    const tags = li.dataset.tags ? li.dataset.tags.split(',') : [];
    tags.push(tag.toLowerCase());
    li.dataset.tags = tags.join(',');
    
    const tagSpan = document.createElement('span');
    tagSpan.className = 'tag';
    tagSpan.textContent = tag;
    li.querySelector('.task-tags').appendChild(tagSpan);
  }
}

searchForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const text = searchText.value.toLowerCase();
  const type = searchType.value;
  const tags = searchTags.value.toLowerCase();
  
  document.querySelectorAll('#taskList li').forEach(li => {
    const matchesText = !text || li.textContent.toLowerCase().includes(text);
    const matchesType = !type || li.classList.contains(type);
    const matchesTags = !tags || (li.dataset.tags && li.dataset.tags.includes(tags));
    li.style.display = matchesText && matchesType && matchesTags ? '' : 'none';
  });
});

resetBtn.addEventListener('click', () => {
  searchText.value = '';
  searchType.value = '';
  searchTags.value = '';
  document.querySelectorAll('#taskList li').forEach(li => {
    li.style.display = '';
  });
});


