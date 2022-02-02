import { invoke } from "@tauri-apps/api";
import styled from "styled-components";
import { IMap } from "../App";

export const Map = ({ preview_url, title, path }: IMap) => {
  const changeMap = () =>
    invoke("change_map", { path }).then((status) => alert(status ? `Map changed` : `Something went wrong`));

  return (
    <Main>
      <Overlay>
        <Title>{title}</Title>
        <Button onClick={changeMap}>Load map</Button>
      </Overlay>
      <Preview preview={preview_url}></Preview>
    </Main>
  );
};

const Main = styled.div`
  position: relative;
  width: 320px;
  height: 180px;
  margin: 10px;
  border-radius: 10px;
`;

const Button = styled.button`
  position: absolute;
  width: 120px;
  left: calc(50% - 60px);
  bottom: 20px;
  height: 36px;
  border-radius: 10px;
  border: none;
  outline: none;
  color: white;
  background: #62c370;
  box-shadow: 0px 4px 32px 0px #62c3707f;
  font-size: 14px;
  font-family: inherit;
  font-weight: 600;
  cursor: pointer;
`;

const Title = styled.div`
  width: calc(100% - 40px);
  margin: 10px 20px;
  text-align: center;
  color: white;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
  user-select: none;
  font-size: 18px;
`;

const Overlay = styled.div`
  display: flex;
  align-items: center;
  background: rgb(0, 0, 0, 0.5);
  backdrop-filter: blur(5px);
  width: 320px;
  height: 180px;
  position: absolute;
  z-index: 100;
  opacity: 0;
  border-radius: 10px;
  transition: opacity 250ms ease-in-out;
  &:hover {
    opacity: 1;
  }
`;

const Preview = styled.div<{ preview: string }>`
  width: 320px;
  height: 180px;
  background-image: url(${({ preview }) => preview});
  background-position: center;
  background-repeat: no-repeat;
  background-size: cover;
  position: relative;
  border-radius: 10px;
`;
