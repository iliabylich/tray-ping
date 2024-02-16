const { invoke } = window.__TAURI__.tauri;

document.querySelector('button').addEventListener('click', () => {
  const newHost = document.querySelector('input').value;
  invoke('change_host', { newHost });
});
