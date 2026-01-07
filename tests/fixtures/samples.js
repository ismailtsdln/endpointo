// Test fixtures - sample JavaScript files for testing

export const sampleMinified = `!function(e){console.log("api call");fetch("https://api.example.com/users").then((function(e){return e.json()})).then((function(e){console.log(e)}))}();`;

export const sampleRestApi = `
const apiClient = {
    baseURL: 'https://api.example.com',
    
    getUsers: function() {
        return fetch('/api/v1/users');
    },
    
    createPost: function(data) {
        return fetch('/api/v1/posts', {
            method: 'POST',
            body: JSON.stringify(data)
        });
    }
};
`;

export const sampleGraphQL = `
import { gql } from '@apollo/client';

export const GET_USER = gql\`
    query GetUser($id: ID!) {
        user(id: $id) {
            id
            name
            email
        }
    }
\`;

const endpoint = '/graphql';
`;

export const sampleWebSocket = `
const ws = new WebSocket('wss://api.example.com/ws');

ws.onmessage = function(event) {
    console.log('Message from server:', event.data);
};
`;
