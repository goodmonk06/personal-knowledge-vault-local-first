import { useState } from 'react';

interface LoginProps {
  onLogin: (password: string) => Promise<void>;
}

export default function Login({ onLogin }: LoginProps) {
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      await onLogin(password);
    } catch (err) {
      setError('データベースの初期化に失敗しました');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="login-container">
      <div className="login-box">
        <h1>Personal Knowledge Vault</h1>
        <p style={{ textAlign: 'center', marginBottom: '20px', color: '#999' }}>
          データベースを暗号化するためのパスワードを入力してください
        </p>
        {error && <div className="error-message">{error}</div>}
        <form onSubmit={handleSubmit}>
          <input
            type="password"
            placeholder="パスワード"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={loading}
            autoFocus
          />
          <button type="submit" disabled={loading || !password}>
            {loading ? '初期化中...' : 'ログイン'}
          </button>
        </form>
      </div>
    </div>
  );
}
