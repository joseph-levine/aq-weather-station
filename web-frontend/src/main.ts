import './style.css'
import Chart from 'chart.js/auto';
import '@picocss/pico/css/pico.classless.min.css';
import {DateTime} from "luxon";
import 'chartjs-adapter-luxon';


const getContrastYIQ = (hexcolor: String) => {
    const r = parseInt(hexcolor.substring(1, 3), 16);
    const g = parseInt(hexcolor.substring(3, 5), 16);
    const b = parseInt(hexcolor.substring(5, 7), 16);
    const yiq = ((r * 299) + (g * 587) + (b * 114)) / 1000;
    return (yiq >= 128) ? 'black' : 'white';
};

interface WeatherData {
    humidity_pct: number;
    pressure_in_hg: number;
    pressure_pascal: number;
    temperature_c: number;
    temperature_f: number;
    timestamp: string;
}

interface AirQualityData {
    aqi_name: string;
    aqi: number;
    aqi_color: string;
    pm10: number;
    pm1_0: number;
    pm2_5: number;
    timestamp: string;
}

(async function () {
    const weatherReq = await fetch('/weather-readings');
    const weatherData: WeatherData[] = await weatherReq.json()
    const latestWeather = weatherData[0];

    const aqReq = await fetch('/air-quality-readings');
    const aqData: AirQualityData[] = await aqReq.json()
    const latestAq = aqData[0];

    document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <h1>Current conditions</h1>
    <h3>${latestAq?.aqi || 'Unknown'} (${latestAq?.pm2_5 || 'Unknown'} μg/m³)</h3>
    <h3 style="background-color: #${latestAq?.aqi_color || 'white'}; color: #${latestAq ? getContrastYIQ(latestAq.aqi_color) : 'black'}">${latestAq?.aqi_name || 'Unknown'}</h3>
    <h3>${latestWeather?.temperature_f?.toFixed(1) || 'Unknown'}˚F (${latestWeather?.temperature_c?.toFixed(1) || 'Unknown'}˚C) ${latestWeather?.humidity_pct?.toFixed(1) || 'Unknown'}% Humidity</h3>
    <h3>${latestWeather?.pressure_in_hg?.toFixed(1) || 'Unknown'} inHg (${(latestWeather?.pressure_pascal / 100)?.toFixed(1) || 'Unknown'} millibars)</h3>
    <canvas id="aqi-chart"></canvas>
    <canvas id="temp-chart"></canvas>
    <canvas id="pressure-chart"></canvas>
    <canvas id="humidity-chart"></canvas>
`;
    const timestamps = weatherData.map(r => DateTime.fromISO(r.timestamp));
    const aqiChartData = {
        labels: aqData.map(r => DateTime.fromISO(r.timestamp)),
        datasets: [
            {label: 'AQI', data: aqData.map(r => r.aqi?.toFixed(0))},
            {label: 'PM 1.0', data: aqData.map(r => r.pm1_0?.toFixed(2))},
            {label: 'PM 2.5', data: aqData.map(r => r.pm2_5?.toFixed(2))},
            {label: 'PM 10', data: aqData.map(r => r.pm10?.toFixed(2))}
        ]
    };
    const tempChartData = {
        labels: timestamps,
        datasets: [{label: 'Temperature', data: weatherData.map(r => r.temperature_f?.toFixed(1))}]
    };
    const pressureChartData = {
        labels: timestamps,
        datasets: [{label: 'Pressure', data: weatherData.map(r => r.pressure_in_hg?.toFixed(1))}]
    };
    const humidityChartData = {
        labels: timestamps,
        datasets: [{label: 'Humidity', data: weatherData.map(r => r.humidity_pct?.toFixed(1))}]
    };
    const shared_options = {
        locale: 'en-US',
        scales: {x: {adapters: {date: {locale: 'en-US' }}, type: 'time', offsetAfterAutoskip: true}}
    };
    new Chart(document.querySelector<HTMLCanvasElement>('#aqi-chart')!, {
        type: 'line',
        data: aqiChartData,
        options: shared_options
    });
    new Chart(document.querySelector<HTMLCanvasElement>('#temp-chart')!, {
        type: 'line',
        data: tempChartData,
        options: shared_options
    });
    new Chart(document.querySelector<HTMLCanvasElement>('#pressure-chart')!, {
        type: 'line',
        data: pressureChartData,
        options: shared_options
    });
    new Chart(document.querySelector<HTMLCanvasElement>('#humidity-chart')!, {
        type: 'line',
        data: humidityChartData,
        options: shared_options
    });
})();