import { JSX } from "solid-js";
import { logout } from "@/routes/logout";
import { Component } from "solid-js";

type LogoutButtonProps = {
  children: JSX.Element;
  onLogout?: () => void;
};

const LogoutButton: Component<LogoutButtonProps> = (props) => {
  const refreshToken = localStorage.getItem("refreshToken");
  
  const handleLogout = async () => {
    await logout(refreshToken);
    props.onLogout?.();
  };

  return <div onClick={handleLogout}>{props.children}</div>;
};

export default LogoutButton;
