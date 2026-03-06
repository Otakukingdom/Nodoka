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
  // Mobile (<640px): Position at bottom-center, above player controls
  // Desktop (>=640px): Position at top-center, below topbar (avoids overlapping panels)
  const isMobile = width < 640;
  const isLandscape = width > height;

  // Safe offsets to prevent overlap with app chrome
  const topBarHeight = isMobile ? 56 : 64;
  const playerControlsHeight = isMobile ? (isLandscape ? 56 : 64) : 72;
  const padding = 16;

  const toastStyle: React.CSSProperties = {
    position: 'fixed',
    // On mobile: position at bottom-center, above player controls
    // On desktop: position at top-center, below topbar
    ...(isMobile
      ? { bottom: playerControlsHeight + padding }
      : { top: topBarHeight + padding }),
    left: '50%',
    transform: 'translateX(-50%)',
    zIndex: 50,
  };

  return (
    <div
      className="toast toast-center gap-3 p-4 max-h-screen overflow-hidden notification-container flex flex-col items-center"
      style={toastStyle}
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
        />
      ))}
    </div>
  );
}

interface NotificationToastProps {
  notification: Notification;
  onDismiss: () => void;
}

function NotificationToast({ notification, onDismiss }: NotificationToastProps) {
  const { bgClass, borderClass, iconColor } = getNotificationStyles(notification.kind);
  const Icon = getIcon(notification.kind);

  return (
    <div
      className={`alert ${bgClass} shadow-xl toast-slide-in min-w-[300px] max-w-md relative overflow-hidden border-l-4 ${borderClass} backdrop-blur-sm border border-base-300/30`}
    >
      {/* Colored accent bar on left */}
      <div className={`absolute left-0 top-0 bottom-0 w-1 ${borderClass.replace('border-l-', 'bg-')}`} />

      <div className="flex items-start gap-3 pl-1">
        <div className={`p-1.5 rounded-full ${iconColor} bg-base-100/50 shadow-sm`}>
          <Icon className="h-4 w-4" />
        </div>
        <div className="flex-1 min-w-0">
          {notification.title && (
            <div className="font-semibold text-sm mb-0.5">{notification.title}</div>
          )}
          <div className="text-sm opacity-90 leading-snug break-words">{notification.message}</div>
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
