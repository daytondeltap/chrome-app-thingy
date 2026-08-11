const invoke = window.__TAURI__.core.invoke;
const els = Object.fromEntries(['fileStatus','importButton','clearButton','width','widthValue','textScale','textValue','opacity','opacityValue','alwaysOnTop','showLabel','openWidget','hideWidget','previewLabel','previewFlag','previewName','previewTime','previewRoom'].map(id=>[id,document.getElementById(id)]));
let state={schedule:null,settings:null};
function applyState(s){
  state=s||state; const set=state.settings||{width:330,text_scale:100,opacity:96,always_on_top:true,show_label:true};
  els.width.value=set.width;els.widthValue.textContent=`${set.width}px`;els.textScale.value=set.text_scale;els.textValue.textContent=`${set.text_scale}%`;els.opacity.value=set.opacity;els.opacityValue.textContent=`${set.opacity}%`;els.alwaysOnTop.checked=!!set.always_on_top;els.showLabel.checked=set.show_label!==false;
  const has=!!state.schedule?.entries?.length;els.fileStatus.textContent=has?`${state.schedule.entries.length} classes imported`:'no schedule imported';els.clearButton.classList.toggle('hidden',!has);
  const m=ScheduleUtils.widgetModel(state.schedule,new Date());els.previewLabel.textContent=m.label;els.previewLabel.classList.toggle('hidden',set.show_label===false);els.previewFlag.textContent=m.flag;els.previewFlag.classList.toggle('hidden',!m.flag);els.previewName.textContent=m.name;els.previewTime.textContent=m.time;els.previewRoom.textContent=m.room;
}
async function refresh(){state=await invoke('get_state');applyState(state)}
async function save(){await invoke('save_settings',{settings:{width:Number(els.width.value),text_scale:Number(els.textScale.value),opacity:Number(els.opacity.value),always_on_top:els.alwaysOnTop.checked,show_label:els.showLabel.checked}});await refresh()}
for(const el of [els.width,els.textScale,els.opacity])el.addEventListener('input',()=>{els.widthValue.textContent=`${els.width.value}px`;els.textValue.textContent=`${els.textScale.value}%`;els.opacityValue.textContent=`${els.opacity.value}%`});
for(const el of [els.width,els.textScale,els.opacity,els.alwaysOnTop,els.showLabel])el.addEventListener('change',save);
els.importButton.onclick=async()=>{const r=await invoke('import_schedule');if(r.ok){await refresh();await invoke('open_widget')}else if(!r.canceled)els.fileStatus.textContent=r.error||'import failed'};
els.clearButton.onclick=async()=>{await invoke('clear_schedule');await refresh()};
els.openWidget.onclick=()=>invoke('open_widget');els.hideWidget.onclick=()=>invoke('hide_widget');
refresh();setInterval(refresh,15000);
