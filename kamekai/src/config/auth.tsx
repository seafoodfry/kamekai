import { User } from 'oidc-client-ts';

export const cognitoConfig = {
  authority: 'https://cognito-idp.us-east-1.amazonaws.com/us-east-1_PlEukB6lN',
  client_id: '7lkjn4ni4rpv1a6co5pgt20p79',
  redirect_uri: 'tauri://localhost', //'tauri://com.kamekai.app/auth/callback',
  logout_uri: 'tauri://localhost', //'tauri://com.kamekai.app/auth/logout',
  domain: 'auth.seafoodfry.ninja',
  // See
  // https://github.com/authts/react-oidc-context/blob/f175dcba6ab09871b027d6a2f2224a17712b67c5/src/AuthProvider.tsx#L20-L30
  // We must provide an implementation of onSigninCallback to oidcConfig to remove the payload from
  // the URL upon successful login. Otherwise if you refresh the page and the payload is still there,
  //  signinSilent - which handles renewing your token - won't work.
  onSigninCallback: (_user: User | void): void => {
    // Remove the OIDC data from the URL
    window.history.replaceState({}, document.title, window.location.pathname);
  },
  // Add automatic token renewal.
  // See https://authts.github.io/oidc-client-ts/interfaces/UserManagerSettings.html#automaticSilentRenew
  automaticSilentRenew: true,
};

export const getLogoutUrl = () =>
  `https://${cognitoConfig.domain}/logout?client_id=${cognitoConfig.client_id}&logout_uri=${encodeURIComponent(cognitoConfig.logout_uri)}`;
