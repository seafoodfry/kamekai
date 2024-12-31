import React from 'react';
import ReactDOM from 'react-dom/client';
import { AuthProvider } from 'react-oidc-context';
import { cognitoConfig } from './config/auth';
import App from './App';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <AuthProvider {...cognitoConfig}>
      <App />
    </AuthProvider>
  </React.StrictMode>
);
