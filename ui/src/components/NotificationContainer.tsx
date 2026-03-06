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
  const { width } = useWindowSize();

  const handleDismiss = (id: number) => {
    dispatch(removeNotification(id));
  };

  // Calculate safe positioning based on viewport.
  // Notifications are centered at the bottom above the player controls
  // to avoid overlapping with content headers and interactive elements.
  const isMobile = width < 640;

  // Player controls height: approximately 72-76px mobile, 88px desktop
  // Account for compact viewport touch targets
  const playerControlsHeight = isMobile ? 76 : 88;
  const bottomPadding = 8;

  // Position toasts centered at bottom above player controls
  // This ensures they never overlap with content headers or main UI elements
  const toastStyle: React.CSSProperties = {
    position: 'fixed',
    zIndex: 45, // Below modals (z-50), above content
    // Max height constraint prevents the toast container from occupying too much space.
    maxHeight: isMobile ? '30vh' : '40vh',
    overflowY: 'auto' as const,
    bottom: playerControlsHeight + bottomPadding,
    left: '50%',
    transform: 'translateX(-50%)',
    maxWidth: isMobile ? 'calc(100vw - 16px)' : '420px',
    width: isMobile ? 'calc(100vw - 16px)' : 'auto',
  };

  // Center-aligned for balanced, non-intrusive placement
  const positionClass = 'items-center';
  const animationClass = 'toast-slide-up';

  return (
    <div
      className={`notification-toast-container gap-2 sm:gap-2.5 p-2 sm:p-3 overflow-y-auto flex flex-col ${positionClass}`}
      style={{
        ...toastStyle,
      }}
      role="region"
      aria-label="Notifications"
      aria-live="polite"
      aria-atomic="false"
      data-testid="notification-container"
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
      className={`alert ${bgClass} shadow-xl ${animationClass} max-w-md relative overflow-hidden border-l-4 ${borderClass} backdrop-blur-sm border border-base-300/30 w-full sm:min-w-[300px] p-2 sm:p-4 toast-notification`}
    >
      {/* Colored accent bar on left */}
      <div className={`absolute left-0 top-0 bottom-0 w-1 ${borderClass.replace('border-l-', 'bg-')}`} />

      <div className="flex items-start gap-1.5 sm:gap-3 pl-1">
        <div className={`p-0.5 sm:p-1.5 rounded-full ${iconColor} bg-base-100/50 shadow-sm shrink-0`}>
          <Icon className="h-3 w-3 sm:h-4 sm:w-4" />
        </div>
        <div className="flex-1 min-w-0">
          {notification.title && (
            <div className="font-semibold text-[11px] sm:text-sm mb-0.5">{notification.title}</div>
          )}
          <div className="text-[11px] sm:text-sm opacity-90 leading-snug break-words">{notification.message}</div>
        </div>
      </div>
      <button
        className="btn btn-ghost btn-circle btn-xs sm:btn-sm hover:bg-base-content/10 hover:scale-110 active:scale-95 transition-all shrink-0 toast-dismiss-btn"
        onClick={onDismiss}
        aria-label="Dismiss notification"
      >
        <X className="h-3.5 w-3.5 sm:h-5 sm:w-5" />
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
