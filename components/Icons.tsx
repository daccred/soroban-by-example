import React from 'react';

export const SorobanSvg: React.FC<React.HTMLAttributes<SVGElement>> = ({ ...props }) => {
    return (
        <span>
            <svg {...props} viewBox="0 0 66 74" xmlns="http://www.w3.org/2000/svg">
                <path d="M19.3376 44.5811L52.2863 68.496C44.8122 72.7602 33.9566 72.3919 27.0666 67.391L3.66116 50.4029C1.44628 48.7952 1.44628 46.1888 3.66116 44.5811C7.99006 41.4391 15.0087 41.4391 19.3376 44.5811Z" fill="currentColor" stroke="currentColor" strokeWidth="4" />
                <path d="M38.5243 6.0489L61.9297 23.0371C64.1446 24.6447 64.1446 27.2512 61.9297 28.8588C57.6008 32.0008 50.5823 32.0008 46.2533 28.8588L13.3047 4.944C20.7788 0.679641 31.6343 1.04795 38.5243 6.0489Z" fill="currentColor" stroke="currentColor" strokeWidth="4" />
                <path d="M57.3051 63.1297L9.45719 28.4007C2.56729 23.3998 2.05979 15.5206 7.93489 10.0957L55.7828 44.8248C62.6727 49.8256 63.1802 57.7049 57.3051 63.1297Z" fill="currentColor" stroke="currentColor" strokeWidth="4" />
            </svg>

        </span>
    )
}

export const OkashiSvg: React.FC<React.HTMLAttributes<SVGElement>> = ({ ...props }) => {
    return (
        <span>
            <svg {...props} viewBox="0 0 26 26" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path fillRule="evenodd" clipRule="evenodd" d="M9.03908 6.46275C10.2297 1.85537 14.9299 -0.914476 19.5372 0.276126C24.1446 1.46673 26.9145 6.16692 25.7239 10.7743C24.7069 14.7097 21.1294 17.3045 17.2363 17.2357C17.2525 18.1924 17.1083 19.168 16.7859 20.1267C15.2694 24.6373 10.3834 27.0645 5.87281 25.5479C1.36221 24.0314 -1.06495 19.1454 0.45161 14.6348C1.67838 10.9861 5.10993 8.70069 8.76408 8.76325C8.75177 8.00503 8.84014 7.23259 9.03908 6.46275ZM8.95478 10.4258C5.9217 10.2777 3.03599 12.1418 2.02044 15.1623C1.32794 17.222 1.65325 19.3785 2.72502 21.0857C3.85862 17.359 6.08446 14.242 9.832 12.7717C9.42947 12.0403 9.13227 11.2502 8.95478 10.4258ZM10.7982 14.1787C6.8307 15.5589 5.13165 18.8613 3.99763 22.5894C4.67656 23.1923 5.48537 23.6715 6.40028 23.9791C10.0444 25.2044 13.9919 23.2434 15.2171 19.5993C15.5023 18.751 15.6149 17.8863 15.5738 17.0433C13.7184 16.6438 12.0224 15.6306 10.7982 14.1787ZM19.1231 1.87862C17.0193 1.33497 14.8916 1.8134 13.2653 3.00426C18.265 4.14237 21.9912 7.49481 23.0585 12.6506C23.5367 11.9775 23.9025 11.2073 24.1214 10.3602C25.0833 6.63785 22.8455 2.84052 19.1231 1.87862ZM10.6416 6.87685C10.8831 5.94227 11.3033 5.10126 11.8563 4.38099C17.2011 5.59766 20.5069 8.48986 21.6689 14.1033C20.0225 15.3884 17.8183 15.9214 15.6398 15.3584C11.9175 14.3965 9.67967 10.5992 10.6416 6.87685Z" fill="currentColor" />
            </svg>
        </span>
    )
}

export const CloseSvg: React.FC<React.HTMLAttributes<SVGElement>> = ({ ...props }) => {
    return (
        <span>
            <svg xmlns="http://www.w3.org/2000/svg" {...props} viewBox="0 0 24 24"><path fill="#888888" d="M6.4 19L5 17.6l5.6-5.6L5 6.4L6.4 5l5.6 5.6L17.6 5L19 6.4L13.4 12l5.6 5.6l-1.4 1.4l-5.6-5.6z" /></svg>
        </span>
    )
}

export const ThreeDotsLoadingSvg: React.FC<React.HTMLAttributes<SVGElement>> = ({ ...props }) => {
    return (
        <span>
            <svg xmlns="http://www.w3.org/2000/svg" {...props} viewBox="0 0 24 24"><circle cx="18" cy="12" r="0" fill="#888888"><animate attributeName="r" begin=".67" calcMode="spline" dur="1.5s" keySplines="0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8" repeatCount="indefinite" values="0;2;0;0" /></circle><circle cx="12" cy="12" r="0" fill="#888888"><animate attributeName="r" begin=".33" calcMode="spline" dur="1.5s" keySplines="0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8" repeatCount="indefinite" values="0;2;0;0" /></circle><circle cx="6" cy="12" r="0" fill="#888888"><animate attributeName="r" begin="0" calcMode="spline" dur="1.5s" keySplines="0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8" repeatCount="indefinite" values="0;2;0;0" /></circle></svg>
        </span>
    )
}

export const SparkleSvg: React.FC<React.HTMLAttributes<SVGElement>> = ({ ...props }) => {
    return (
        <span>
            <svg xmlns="http://www.w3.org/2000/svg" {...props} viewBox="0 0 24 24"><path fill="none" stroke="#888888" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0zM20 3v4m2-2h-4M4 17v2m1-1H3" /></svg>
        </span>
    )
}

export const CopySvg: React.FC<React.HTMLAttributes<SVGElement>> = ({ ...props }) => {
    return (
        <span>
            <svg xmlns="http://www.w3.org/2000/svg" {...props} viewBox="0 0 24 24"><path fill="#888888" d="M9 18q-.825 0-1.412-.587T7 16V4q0-.825.588-1.412T9 2h9q.825 0 1.413.588T20 4v12q0 .825-.587 1.413T18 18zm0-2h9V4H9zm-4 6q-.825 0-1.412-.587T3 20V6h2v14h11v2zm4-6V4z"/></svg>
        </span>
    )
}
