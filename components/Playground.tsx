import React, { useState } from 'react';
import { OkashiSvg, SorobanSvg } from './Icons';

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

const buttonStyle = {
  position: 'absolute' as const,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  gap: '0.5rem',
  top: '0',
  right: '0',
  backgroundColor: "#312f36",
  padding: '0.5rem 1.3rem',
  cursor: 'pointer',
  fontSize: '1rem',
  fontWeight: 600,
  color: '#fff',
  transition: 'background 0.2s ease-in-out',
  '&:hover': {
    backgroundColor: 'rgba(0,0,0,0)'
  }
}


export const Playground: React.FC<PlaygroundProps> = ({ okashiID }) => {
  const [isLoading, setIsLoading] = useState(true);

  return (
    <div style={wrapperStyle}>
      <button >OpenOkashi</button>
      {isLoading && (
        <div style={loadingOverlayStyle}>
          <p style={loadingTextStyle}>Loading playground...</p>
        </div>
      )}

      <div>
        <a href={`https://okashi.dev/playground/${okashiID}`} target='_blank' style={buttonStyle}>
          <span>
            <OkashiSvg style={{width: '20px', height: '24px', color: '#fff' }}/>
          </span>
          <span>
            Run on Okashi
          </span>
        </a>
      </div>

      <iframe
        width="100%" height="600px"
        src={`https://okashi-proxy-production.up.railway.app/playground/${okashiID}`}
        onLoad={() => setTimeout(() => setIsLoading(false), 1000)}
        title="Okashi Playground"
      />
    </div>
  )
}