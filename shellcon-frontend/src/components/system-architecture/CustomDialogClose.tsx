
import React from 'react';
import { X } from 'lucide-react';
import { DialogClose } from '@/components/ui/dialog';
import { cn } from '@/lib/utils';

/**
 * Custom close button for dialogs that removes the focus ring
 */
export const CustomDialogClose: React.FC<React.ComponentPropsWithoutRef<typeof DialogClose>> = ({ 
  className,
  ...props 
}) => (
  <DialogClose
    className={cn(
      "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 outline-none focus:outline-none focus:ring-0 focus-visible:ring-0 focus-visible:outline-none disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground",
      className
    )}
    {...props}
  >
    <X className="h-4 w-4" />
    <span className="sr-only">Close</span>
  </DialogClose>
);
