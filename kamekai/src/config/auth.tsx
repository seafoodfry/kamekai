export const cognitoConfig = {
  authority: 'https://cognito-idp.us-east-1.amazonaws.com/us-east-1_ES1AIyLI9',
  client_id: '6351vlk6qmcro8jde3onl3ug8h',
  redirect_uri: 'tauri://localhost', //'tauri://com.kamekai.app/auth/callback',
  logout_uri: 'tauri://localhost', //'tauri://com.kamekai.app/auth/logout',
  domain: 'auth.seafoodfry.ninja',
};

export const getLogoutUrl = () =>
  `https://${cognitoConfig.domain}/logout?client_id=${cognitoConfig.client_id}&logout_uri=${encodeURIComponent(cognitoConfig.logout_uri)}`;
