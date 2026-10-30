import { useState } from 'react';
import './content.css';

interface DayItem {
  dateObj: Date;
  dayName: string; // z. B. "Mo", "Di"
  formattedDate: string; // z. B. "01.03."
  isoString: string; // Zum eindeutigen Vergleichen (YYYY-MM-DD)
}

function Content() {



    // Generiert z. B. die letzten 30 Tage bis HEUTE
    const generateDays = (count: number): DayItem[] => {
        const list: DayItem[] = [];
        const today = new Date();

        for (let i = count - 1; i >= 0; i--) {
        const d = new Date();
        d.setDate(today.getDate() - i);

        const dayName = d.toLocaleDateString('de-DE', { weekday: 'short' }); // Mo, Di, Mi...
        const formattedDate = d.toLocaleDateString('de-DE', {
            day: '2-digit',
            month: '2-digit',
        }) + '.'; // 01.03.

        list.push({
            dateObj: d,
            dayName,
            formattedDate,
            isoString: d.toISOString().split('T')[0],
        });
        }

        return list;
    };

  // 30 Tage erstellen
  const [days] = useState<DayItem[]>(() => generateDays(30));
  // Standardmäßig ist HEUTE ausgewählt (der letzte Tag in der Liste)
  const [selectedIso, setSelectedIso] = useState<string>(
    days[days.length - 1].isoString
  );

  return (
    <div className="date-scroll-container">
      {days.map((item) => {
        const isSelected = item.isoString === selectedIso;
        return (
          <button
            key={item.isoString}
            className={`date-card ${isSelected ? 'active' : ''}`}
            onClick={() => setSelectedIso(item.isoString)}
          >
            <span className="day-name">{item.dayName}</span>
            <span className="day-date">{item.formattedDate}</span>
          </button>
        );
      })}
    </div>
  );
}


export default Content