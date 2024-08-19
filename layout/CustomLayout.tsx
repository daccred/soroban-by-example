import React, { ReactNode } from 'react';
import SoroboyWidget from '../components/SoroboyWidget';

type CustomLayoutProps = {
    children: ReactNode;
};

export const CustomLayout = ({ children }: CustomLayoutProps) => {
    return (
        <div className='relative'>
            <SoroboyWidget />
            {children}
        </div>
    );
};