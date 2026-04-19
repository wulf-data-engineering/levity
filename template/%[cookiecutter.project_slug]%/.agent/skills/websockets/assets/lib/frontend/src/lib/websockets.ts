import { loadConfig } from '$lib/config';
import { fetchAuthSession } from 'aws-amplify/auth';

/**
 * Helper to fetch the websocket URL from config
 * 
 * @returns The fully qualified websocket url to connect to
 */
export async function getWebSocketUrl(): Promise<string> {
    const config = await loadConfig();
    return config.webSocketUrl || '';
}

/**
 * Establishes a WebSocket connection. It authenticates the connection by passing the
 * current user's Cognito Access Token through the Sec-WebSocket-Protocol header so that
 * the AWS API Gateway authorizer lambda can validate it.
 *
 * @param topicId The unique topic topic to connect to (will be appended as ?topicId=...)
 * @returns A fresh WebSocket instance
 */
export async function connectWebSocket(topicId: string): Promise<WebSocket> {
    const url = await getWebSocketUrl();
    const session = await fetchAuthSession();
    const accessToken = session.tokens?.accessToken?.toString() || '';

    // Pass the access token as the Sec-WebSocket-Protocol so API Gateway can read it
    const ws = new WebSocket(`${url}?topicId=${topicId}`, accessToken);
    
    return ws;
}
