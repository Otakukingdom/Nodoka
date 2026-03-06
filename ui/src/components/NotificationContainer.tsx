/**
 * Notification container component showing toast notifications
 */

import { X, CheckCircle, Info, AlertTriangle, AlertCircle } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { removeNotification } from '../store/slices/appSlice';
import type { Notification, NotificationKind } from '../store/types';
import { useWindowSize } from '../hooks/useWindowSize';

export default function NotificationContainer() {
  const { notifications } = useAppSelector(state => state.app);
  const dispatch = useAppDispatch();
  const { width, height } = useWindowSize();

  const handleDismiss = (id: number) => {
    dispatch(removeNotification(id));
  };

  // Calculate safe positioning based on viewport
  // Mobile (<640px) Portrait: Position at bottom-center, above player controls
  // Mobile (<640px) Landscape: Position at top-right to avoid player controls overlap
  // Desktop (>=640px): Position at top-right to avoid overlapping content panels
  const isMobile = width < 640;
  const isLandscape = width > height;
  const isMobileLandscape = isMobile && isLandscape;

  // Safe offsets to prevent overlap with app chrome
  const topBarHeight = isMobile ? 56 : 64;
  const playerControlsHeight = isMobile ? (isLandscape ? 48 : 64) : 72;
  const padding = 16;
  const sidePadding = 16;

  // Position toasts to avoid overlapping core content:
  // - Desktop: Top-right corner (doesn't block library/details panels)
  // - Mobile Portrait: Bottom-center above player controls with max-height constraint
  // - Mobile Landscape: Bottom positioning above player controls (avoids panel content overlap)
  const toastStyle: React.CSSProperties = {
    position: 'fixed',
    zIndex: 45, // Below modals (z-50), above content
    // Max height constraint prevents toast container from occupying too much screen space
    // Enables scrolling when multiple toasts stack
    maxHeight: isMobile ? '40vh' : '60vh',
    overflowY: 'auto' as const,
    ...(isMobileLandscape
      ? {
          // Mobile landscape: bottom positioning to avoid panel content overlap
          // Spans width with safe margins, sits above player controls
          bottom: playerControlsHeight + padding / 2,
          left: sidePadding,
          right: sidePadding,
          maxWidth: '100%',
        }
      : isMobile
        ? {
            // Mobile portrait: bottom-center above player
            bottom: playerControlsHeight + padding,
            left: '50%',
            transform: 'translateX(-50%)',
            maxWidth: 'calc(100vw - 32px)',
          }
        : {
            // Desktop: top-right corner
            top: topBarHeight + padding,
            right: sidePadding,
            maxWidth: '480px',
          }),
    width: isMobile && !isMobileLandscape ? 'auto' : undefined,
  };

  // Determine animation class based on position
  // Right-aligned: slide in from right
  // Center-aligned (mobile portrait): slide up from bottom
  const positionClass = isMobile && !isMobileLandscape
    ? 'toast-center items-center'
    : 'items-end';
  const animationClass = isMobile && !isMobileLandscape
    ? 'toast-slide-up'
    : 'toast-slide-in';

  return (
    <div
      className={`toast gap-2 sm:gap-3 p-3 sm:p-4 overflow-y-auto notification-container flex flex-col ${positionClass}`}
      style={{ ...toastStyle, maxHeight: isMobile ? '40vh' : '50vh' }}
      role="region"
      aria-label="Notifications"
      aria-live="polite"
      aria-atomic="false"
    >
      {notifications.map(notification => (
        <NotificationToast
          key={notification.id}
          notification={notification}
          onDismiss={() => handleDismiss(notification.id)}
          animationClass={animationClass}
        />
      ))}
    </div>
  );
}

interface NotificationToastProps {
  notification: Notification;
  onDismiss: () => void;
  animationClass?: string;
}

function NotificationToast({ notification, onDismiss, animationClass = 'toast-slide-in' }: NotificationToastProps) {
  const { bgClass, borderClass, iconColor } = getNotificationStyles(notification.kind);
  const Icon = getIcon(notification.kind);

  return (
    <div
      className={`alert ${bgClass} shadow-xl ${animationClass} max-w-md relative overflow-hidden border-l-4 ${borderClass} backdrop-blur-sm border border-base-300/30 w-full sm:min-w-[300px] p-3 sm:p-4`}
    >
      {/* Colored accent bar on left */}
      <div className={`absolute left-0 top-0 bottom-0 w-1 ${borderClass.replace('border-l-', 'bg-')}`} />

      <div className="flex items-start gap-2 sm:gap-3 pl-1">
        <div className={`p-1 sm:p-1.5 rounded-full ${iconColor} bg-base-100/50 shadow-sm shrink-0`}>
          <Icon className="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        </div>
        <div className="flex-1 min-w-0">
          {notification.title && (
            <div className="font-semibold text-xs sm:text-sm mb-0.5">{notification.title}</div>
          )}
          <div className="text-xs sm:text-sm opacity-90 leading-snug break-words">{notification.message}</div>
        </div>
      </div>
      <button
        className="btn btn-ghost btn-sm btn-circle hover:bg-base-content/10 hover:scale-110 active:scale-95 transition-all shrink-0 min-w-[44px] min-h-[44px]"
        onClick={onDismiss}
        aria-label="Dismiss"
      >
        <X className="h-4 w-4" />
      </button>
    </div>
  );
}

function getNotificationStyles(kind: NotificationKind): { bgClass: string; borderClass: string; iconColor: string } {
  switch (kind) {
    case 'Success':
      return {
        bgClass: 'bg-success/20',
        borderClass: 'border-l-success',
        iconColor: 'text-success',
      };
    case 'Info':
      return {
        bgClass: 'bg-info/20',
        borderClass: 'border-l-info',
        iconColor: 'text-info',
      };
    case 'Warning':
      return {
        bgClass: 'bg-warning/20',
        borderClass: 'border-l-warning',
        iconColor: 'text-warning',
      };
    case 'Error':
      return {
        bgClass: 'bg-error/20',
        borderClass: 'border-l-error',
        iconColor: 'text-error',
      };
    default:
      return {
        bgClass: 'bg-base-200',
        borderClass: 'border-l-base-300',
        iconColor: 'text-base-content',
      };
  }
}

function getIcon(kind: NotificationKind): typeof CheckCircle {
  switch (kind) {
    case 'Success':
      return CheckCircle;
    case 'Info':
      return Info;
    case 'Warning':
      return AlertTriangle;
    case 'Error':
      return AlertCircle;
    default:
      return Info;
  }
}
