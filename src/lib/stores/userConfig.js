import { SettingsManager } from 'tauri-settings';

export const settingsManager = new SettingsManager(
    {
        theme: 'light',
        language: 'en',
        audio_player: {
            volume: 80,
        },
    },
    {
        fileName: 'config',
        prettify: true,
        numSpaces: 4,
    }
);

export const settings = await settingsManager.initialize();
