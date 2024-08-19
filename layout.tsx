import React from 'react';
import { CustomLayout } from "./layout/index"

export default function Layout({ children }: { children: React.ReactNode }) {
  return <CustomLayout>{children}</CustomLayout>
}