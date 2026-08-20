const { invoke } = window.__TAURI__.core;

const notEl = document.getElementById('note');

const saveNote = debounce(async (content) => {
  try{
    await invoke('save_note', {content});
  } catch(err) {
    console.error('Gagal menyimpan: ', err);
  }
}, 500);

notEl.addEventListener('input', (e)=>{
  saveNote(e.target.value);
});

function debounce(fn, delay){
  let timeOutId;
  return function(...args){
    clearTimeout(timeOutId);
    timeOutId = setTimeout(() => fn(...args), delay);
  };
}

window.addEventListener('DOMContentLoaded', async () => {
  try{
    const data = await invoke('load_note');
    notEl.value = data.content;
  } catch(err){
    console.error('Gagal memuat catatan: ', err);
  }
});