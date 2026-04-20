import { dev } from '$app/environment';

export type Config = {
    userPoolId: string;
    userPoolClientId: string;
    endpoint?: string;
};

let cachedConfig: Config | null = null;
let configPromise: Promise<Config> | null = null;

/**
 * Loads the configuration from /config.json (AWS) or environment variables (dev, defaults to local Cognito endpoint).
 */
export async function loadConfig(): Promise<Config> {
    if (cachedConfig) return cachedConfig;
    if (configPromise) return configPromise;

    configPromise = (async () => {
        if (dev) {
            cachedConfig = {
                userPoolId: import.meta.env.VITE_USER_POOL_ID || 'local_userPool',
                userPoolClientId: import.meta.env.VITE_USER_POOL_CLIENT_ID || 'local_userPoolClient',
                endpoint: import.meta.env.VITE_COGNITO_ENDPOINT || `http://localhost:${import.meta.env.VITE_COGNITO_LOCAL_PORT || '9229'}`
            };
        } else {
            const response = await fetch('/config.json');
            if (!response.ok) throw new Error(`Failed to load config: ${response.statusText}`);
            cachedConfig = await response.json();
        }
        return cachedConfig as Config;
    })();

    return configPromise;
}
