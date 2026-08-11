function localISO(date = new Date()) {
  return `${date.getFullYear()}-${String(date.getMonth()+1).padStart(2,'0')}-${String(date.getDate()).padStart(2,'0')}`;
}
function formatMinutes(minutes) {
  const total = ((minutes % 1440) + 1440) % 1440;
  const h24 = Math.floor(total / 60), min = total % 60;
  const ap = h24 >= 12 ? 'PM' : 'AM';
  return `${h24 % 12 || 12}:${String(min).padStart(2,'0')} ${ap}`;
}
function prettyDate(iso) {
  const [y,m,d] = iso.split('-').map(Number);
  const dt = new Date(y, m - 1, d);
  return `${['Sun','Mon','Tue','Wed','Thu','Fri','Sat'][dt.getDay()]} ${m}/${d}`;
}
function nextClass(schedule, now = new Date()) {
  const entries = schedule?.entries || [];
  if (!entries.length) return null;
  const today = localISO(now), minute = now.getHours() * 60 + now.getMinutes();
  const sorted = [...entries].sort((a,b) => a.date.localeCompare(b.date) || a.start - b.start);
  const current = sorted.find(e => e.date === today && e.start <= minute && minute < e.end);
  if (current) return { entry: current, state: 'current' };
  const next = sorted.find(e => e.date > today || (e.date === today && e.start > minute));
  return next ? { entry: next, state: 'next' } : null;
}
function widgetModel(schedule, now = new Date()) {
  if (!schedule?.entries?.length) return { label:'NO SCHEDULE', name:'import a schedule file', time:'-', room:'-', flag:'' };
  const result = nextClass(schedule, now);
  if (!result) return { label:'NO UPCOMING CLASS', name:'schedule finished', time:'-', room:'-', flag:'' };
  const { entry, state } = result;
  const today = localISO(now);
  return {
    label: state === 'current' ? 'CURRENT CLASS' : 'NEXT CLASS',
    name: entry.name,
    time: `${entry.date === today ? '' : prettyDate(entry.date) + ' · '}${formatMinutes(entry.start)} - ${formatMinutes(entry.end)}`,
    room: entry.room || 'Room not listed',
    flag: (schedule.flags?.[entry.date] || []).join(' / ')
  };
}
window.ScheduleUtils = { localISO, formatMinutes, prettyDate, nextClass, widgetModel };
