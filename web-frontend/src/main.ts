import './style.css'
import Chart from 'chart.js/auto';
import '@picocss/pico/css/pico.classless.min.css';

const getContrastYIQ = (hexcolor: String) => {
    const r = parseInt(hexcolor.substring(1, 3), 16);
    const g = parseInt(hexcolor.substring(3, 5), 16);
    const b = parseInt(hexcolor.substring(5, 7), 16);
    const yiq = ((r * 299) + (g * 587) + (b * 114)) / 1000;
    return (yiq >= 128) ? 'black' : 'white';
};

(async function () {
    const req = await fetch('/readings');
    const response: any[] = await req.json()
    const latest = response[0];

    document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <h1>Current conditions</h1>
    <h3>${latest.aqi} (${latest.pm_25_ug_cubic_m} μg/m³)</h3>
    <h3 style="background-color: #${latest.aqi_color}; color: #${getContrastYIQ(latest.aqi_color)}">${latest.aqi_name}</h3>
    <h3>${latest.temperature_f}˚F (${latest.temperature_c}˚C) ${latest.humidity_pct}% Humidity</h3>
    <h3>${latest.pressure_in} inHg (${latest.pressure_mbar} mbar)</h3>
    <canvas id="aqi_chart"></canvas>
    <canvas id="temp_chart"></canvas>
    <canvas id="pressure_chart"></canvas>
    <canvas id="humidity_chart"></canvas>
`;
    const timestamps = response.map(r => r.utc_datetime);
    const aqi_data = {
        labels: timestamps,
        datasets: [{label: 'AQI', data: response.map(r => r.aqi)}]
    };
    const temp_data = {
        labels: timestamps,
        datasets: [{label: 'Temperature', data: response.map(r => r.temperature_f)}]
    };
    const pressure_data = {
        labels: timestamps,
        datasets: [{label: 'Pressure', data: response.map(r => r.pressure_in)}]
    };
    const humidity_data = {
        labels: timestamps,
        datasets: [{label: 'Humidity', data: response.map(r => r.humidity_pct)}]
    };
    new Chart(document.querySelector<HTMLCanvasElement>('#aqi_chart')!, {type: 'line', data: aqi_data});
    new Chart(document.querySelector<HTMLCanvasElement>('#temp_chart')!, {type: 'line', data: temp_data});
    new Chart(document.querySelector<HTMLCanvasElement>('#pressure_chart')!, {type: 'line', data: pressure_data});
    new Chart(document.querySelector<HTMLCanvasElement>('#humidity_chart')!, {type: 'line', data: humidity_data});
})();