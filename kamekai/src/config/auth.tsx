export const cognitoConfig = {
  authority: 'https://cognito-idp.us-east-1.amazonaws.com/us-east-1_G0l5wgf1W',
  client_id: '1ti7du87tqnv03glf0odrtj4dj',
  redirect_uri: 'http://localhost:1420/auth/callback',
  logout_uri: 'http://localhost:1420/auth/logout',
  domain: 'auth.seafoodfry.ninja',
};

export const getLogoutUrl = () =>
  `https://${cognitoConfig.domain}/logout?client_id=${cognitoConfig.client_id}&logout_uri=${encodeURIComponent(cognitoConfig.logout_uri)}`;
