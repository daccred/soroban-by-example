import React, { useState } from 'react';

type PlaygroundProps = {
  okashiID: string
}

// style variable
const wrapperStyle = {
  width: '600px',
  height: '600px',
  position: 'relative' as const
}

const loadingOverlayStyle = {
  position: 'absolute' as const,
  inset: 0,
  width: '600px',
  height: '600px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  backgroundColor: '#131417'
}

const loadingTextStyle = {
  fontSize: '1.125rem',
  fontWeight: 600,
  color: '#4B5563'
}


export const Playground: React.FC<PlaygroundProps> = ({ okashiID }) => {
  const [isLoading, setIsLoading] = useState(true);

  return (
    <div style={wrapperStyle}>
      {isLoading && (
        <div style={loadingOverlayStyle}>
          <p style={loadingTextStyle}>Loading playground...</p>
        </div>
      )}

      <iframe
        width="100%" height="600px"
        src={`https://okashi-proxy-production.up.railway.app/playground/${okashiID}`}
        onLoad={() => setTimeout(() => setIsLoading(false), 1000)}
        title="Okashi Playground"
      />
    </div>
  )
}