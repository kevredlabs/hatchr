import { useEffect, useState } from 'react';

export function App() {
  const [health, setHealth] = useState<string>('...');

  useEffect(() => {
    const backendUrl = import.meta.env.VITE_BACKEND_URL || 'http://localhost:4000';
    fetch(`${backendUrl}/health`).then(async (r) => setHealth(await r.text())).catch(() => setHealth('unreachable'));
  }, []);

  return (
    <div style={{ fontFamily: 'ui-sans-serif, system-ui', padding: 24 }}>
      <h1>Hatchr</h1>
      <p>Backend health: <strong>{health}</strong></p>
      <p>Edit <code>client/src/App.tsx</code> to get started.</p>
    </div>
  );
}

