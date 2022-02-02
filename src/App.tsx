import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import styled from "styled-components";
import { Map } from "./components/Map";

export interface IMap {
  title: string;
  preview_url: string;
  path: string;
}

function App() {
  const [maps, setMaps] = useState<IMap[]>([]);

  useEffect(() => {
    listen("update-maps", (event) => {
      setMaps(event.payload as IMap[]);
    });

    invoke("init");
    invoke("watch_maps");
  }, []);

  return (
    <Main>
      {maps.map((map) => (
        <Map key={map.path} {...map} />
      ))}
    </Main>
  );
}

const Main = styled.div`
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  background: #000;
  min-height: 100vh;
`;

export default App;
