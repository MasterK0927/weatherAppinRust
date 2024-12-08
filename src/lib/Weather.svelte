<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    interface Geo {
        country: string;
        latitude: number;
        longitude: number;
    }

    interface WeatherInfo {
        timezone: string;
        city_name: string;
        country_code: string;
        datetime: string;
        temp?: number;
        app_temp?: number;
        description: string;
        vis?: number;
    }

    let countries: Geo[] = [];
    let selectedCountry: Geo | null = null;
    let weatherInfo: WeatherInfo | null = null;
    let error: string | null = null;

    onMount(async () => {
        try {
            countries = await invoke('get_geo_info');
            if (countries.length > 0) {
                selectedCountry = countries[0];
            }
        } catch (e) {
            console.error(e);
            error = 'Failed to fetch geo info';
        }
    });

    async function fetchWeather() {
        if (!selectedCountry) return;
        try {
            weatherInfo = await invoke('get_weather', {
                lat: selectedCountry.latitude,
                long: selectedCountry.longitude
            });
            error = null;
        } catch (e) {
            console.error(e);
            error = 'Failed to fetch weather info';
        }
    }

    function selectCountry(event: Event) {
        const selectedIndex = (event.target as HTMLSelectElement).selectedIndex;
        selectedCountry = countries[selectedIndex];
        fetchWeather();
    }
</script>

<style>
    .container {
        max-width: 600px;
        margin: auto;
        padding: 20px;
        background-color: #f0f0f0;
        border-radius: 8px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    }

    h2 {
        font-size: 24px;
        margin-bottom: 16px;
        color: #333;
    }

    select {
        width: 100%;
        padding: 10px;
        font-size: 16px;
        margin-bottom: 16px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .weather-details {
        margin-top: 20px;
        padding: 16px;
        background-color: #fff;
        border-radius: 8px;
        box-shadow: 0 0 5px rgba(0, 0, 0, 0.1);
    }

    .weather-details h3 {
        font-size: 20px;
        margin-bottom: 12px;
        color: #555;
    }

    .weather-details p {
        font-size: 16px;
        margin-bottom: 8px;
        color: #777;
    }
</style>

{#if error}
    <div class="container">
        <p>{error}</p>
    </div>
{:else}
    <div class="container">
        <h2>Select a Country</h2>
        <select on:change={selectCountry} bind:value={selectedCountry}>
            {#each countries as country}
                <option value="{country.country}">{country.country}</option>
            {/each}
        </select>

        {#if weatherInfo}
            <div class="weather-details">
                <h3>Weather Details for {weatherInfo.city_name}, {weatherInfo.country_code}</h3>
                <p>Timezone: {weatherInfo.timezone}</p>
                <p>Date & Time: {weatherInfo.datetime}</p>
                <p>Temperature: {weatherInfo.temp !== undefined ? weatherInfo.temp + '°C' : 'N/A'}</p>
                <p>Feels Like: {weatherInfo.app_temp !== undefined ? weatherInfo.app_temp + '°C' : 'N/A'}</p>
                <p>Description: {weatherInfo.description}</p>
                <p>Visibility: {weatherInfo.vis !== undefined ? weatherInfo.vis + ' meters' : 'N/A'}</p>
            </div>
        {/if}
    </div>
{/if}
