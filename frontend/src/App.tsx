import "./App.css"
import { useState, useEffect } from 'react';
import DeckGL from '@deck.gl/react/typed';
import { ScatterplotLayer, LineLayer } from '@deck.gl/layers/typed';
import Map from 'react-map-gl/maplibre';

// Define your initial view state
const INITIAL_VIEW_STATE = {
    longitude: 16.37, // Center of Vienna
    latitude: 48.2,
    zoom: 11,
    pitch: 0,
    bearing: 0
};

type MeterDto = {
    feeder_id: number,
    id: number,
    position: {
        coordinates: [number, number]
    },
    type: string
}

type LineDto = {
    feeder_id: number,
    line: {
        coordinates: number[][] // Assuming it's an array of coordinate pairs
    }
}


function App() {
    const [meters, setMeters] = useState([]);
    const [lines, setLines] = useState([]);

    // Define a color map for feeder IDs
    const feederColorMap: Record<number, [number, number, number, number]> = {
        1: [255, 0, 0, 200],   // Red for feeder ID 1
        2: [0, 255, 0, 200],   // Green for feeder ID 2
        3: [0, 0, 255, 200],   // Blue for feeder ID 3
        4: [255, 255, 0, 200], // Yellow for feeder ID 4
    };

    useEffect(() => {
        fetch('/api/meters')
            .then(response => response.json())
            .then(data => {
                const formattedData = data.map((item: MeterDto) => ({
                    position: item.position.coordinates,
                    id: item.id,
                    feeder_id: item.feeder_id
                }));
                setMeters(formattedData);
            })
            .catch(error => console.error('Error fetching data:', error));

        // Fetch line data
        fetch('/api/connections')
            .then(response => response.json())
            .then(data => {
                // Process and set line data
                const formattedLines = data.map((item: LineDto) => ({
                    to: item.line.coordinates[0],
                    from: item.line.coordinates[1],
                    feeder_id: item.feeder_id
                }));
                console.log(formattedLines);
                setLines(formattedLines);
            })
            .catch(error => console.error('Error fetching line data:', error));
    }, []);

    const layers = [
        new LineLayer({
            id: 'line-layer',
            data: lines,
            getWidth: 5, // Adjust as needed
            getColor: d => feederColorMap[d.feeder_id], // Default color
            getSourcePosition: d => d.from,
            getTargetPosition: d => d.to,
            pickable: true,
        }),
        new ScatterplotLayer({
            id: 'scatterplot-layer',
            data: meters,
            getPosition: d => d.position,
            getRadius: 500, // Adjust as needed
            getFillColor: d => feederColorMap[d.feeder_id], // Adjust as needed
            pickable: true
            // ... other properties
        })
    ];

    return (
        <DeckGL
            initialViewState={INITIAL_VIEW_STATE}
            layers={layers}
            controller={true}
            getTooltip={({object}) => object && `Id: ${object.id}\nFeeder ${object.feeder_id}`}
        >
            <Map mapStyle={"http://localhost:8070/styles/light/style.json"} />
        </DeckGL>
    );
}

export default App;
