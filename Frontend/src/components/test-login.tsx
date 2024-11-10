import { createSignal } from 'solid-js';
import { userLogin, facilityLogin, organiserLogin, adminLogin } from '@/utils/login';


function LoginButtons() {
  const [icNumber, setIcNumber] = createSignal('');
  const [email, setEmail] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [loginResult, setLoginResult] = createSignal<string | null>(null);

  const handleUserLogin = async () => {
    try {
      const result = await userLogin(icNumber(), password());
    } catch (error) {
      setLoginResult('User login failed');
    }
  };

  const handleFacilityLogin = async () => {
    try {
      const result = await facilityLogin(email(), password());
      setLoginResult('Facility login successful!');
    } catch (error) {
      setLoginResult('Facility login failed');
    }
  };

  const handleOrganiserLogin = async () => {
    try {
      const result = await organiserLogin(email(), password());
      setLoginResult('Organiser login successful!');
    } catch (error) {
      setLoginResult('Organiser login failed');
    }
  };

  const handleAdminLogin = async () => {
    try {
      const result = await adminLogin(email(), password());
      setLoginResult('Admin login successful!');
    } catch (error) {
      setLoginResult('Admin login failed');
    }
  };

  return (
    <div>
      <input
        type="text"
        placeholder="IC Number"
        onInput={(e) => setIcNumber(e.currentTarget.value)}
        value={icNumber()}
      />
      <input
        type="email"
        placeholder="Email"
        onInput={(e) => setEmail(e.currentTarget.value)}
        value={email()}
      />
      <input
        type="password"
        placeholder="Password"
        onInput={(e) => setPassword(e.currentTarget.value)}
        value={password()}
      />
      
      <button onClick={handleUserLogin}>User Login</button>
      <button onClick={handleFacilityLogin}>Facility Login</button>
      <button onClick={handleOrganiserLogin}>Organiser Login</button>
      <button onClick={handleAdminLogin}>Admin Login</button>
      
      {loginResult() && <p>{loginResult()}</p>}
    </div>
  );
};

export default LoginButtons;
