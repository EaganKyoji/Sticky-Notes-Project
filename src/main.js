const { invoke } = window.__TAURI__.core;
const { getCurrentWindow, PhysicalPosition, PhysicalSize} = window.__TAURI__.window;
const  appWindow = getCurrentWindow();

const saveConfig = debounce(async () => {
  const position = await appWindow.outerPosition();
  const size = await appWindow.outerSize();
  try{
    await invoke('save_config', {
      x: position.x,
      y: position.y,
      width: size.width,
      height: size.height,
    });
  } catch(err){
    console.error("Gagal simpan posisi/ukuran: ", err);
  }
}, 500);

appWindow.onMoved(() => saveConfig());
appWindow.onResized(() => saveConfig());

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

window.addEventListener('DOMContentLoaded', async () => {
  try {
    const noteData = await invoke('load_note');
    notEl.value = noteData.content;

    const config = await invoke('load_config');
    await appWindow.setPosition(new PhysicalPosition(config.x, config.y));
    await appWindow.setSize(new PhysicalSize(config.width, config.height));
  } catch (err) {
    console.error('Gagal memuat data:', err);
  }
});