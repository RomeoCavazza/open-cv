document.querySelectorAll('footer p').forEach(p => {
  p.addEventListener('mouseenter', () => p.classList.add('blue'));
  p.addEventListener('mouseleave', () => p.classList.remove('blue'));
  p.addEventListener('click', () => p.classList.toggle('highlighted'));
});


