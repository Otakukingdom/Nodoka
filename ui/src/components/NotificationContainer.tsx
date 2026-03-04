/**
 * Notification container component showing toast notifications
 */

import { X, CheckCircle, Info, AlertTriangle, AlertCircle } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { removeNotification } from '../store/slices/appSlice';
import type { Notification, NotificationKind } from '../store/types';

export default function NotificationContainer() {
  const { notifications } = useAppSelector(state => state.app);
  const dispatch = useAppDispatch();

  const handleDismiss = (id: number) => {
    dispatch(removeNotification(id));
  };

  return (
    <div
      className="toast toast-top toast-end z-50 gap-3 p-4"
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
      className={`alert ${bgClass} shadow-xl toast-slide-in min-w-[300px] max-w-md relative overflow-hidden border-l-4 ${borderClass}`}
    >
      {/* Colored accent bar on left */}
      <div className={`absolute left-0 top-0 bottom-0 w-1 ${borderClass.replace('border-l-', 'bg-')}`} />

      <div className="flex items-start gap-3 pl-1">
        <div className={`p-1 rounded-full ${iconColor} bg-base-100/30`}>
          <Icon className="h-4 w-4" />
        </div>
        <div className="flex-1 min-w-0">
          {notification.title && (
            <div className="font-semibold text-sm mb-0.5">{notification.title}</div>
          )}
          <div className="text-sm opacity-90">{notification.message}</div>
        </div>
      </div>
      <button
        className="btn btn-ghost btn-sm btn-circle hover:bg-base-content/10 transition-colors shrink-0"
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
        bgClass: 'bg-success/15 border-success',
        borderClass: 'border-l-success',
        iconColor: 'text-success',
      };
    case 'Info':
      return {
        bgClass: 'bg-info/15 border-info',
        borderClass: 'border-l-info',
        iconColor: 'text-info',
      };
    case 'Warning':
      return {
        bgClass: 'bg-warning/15 border-warning',
        borderClass: 'border-l-warning',
        iconColor: 'text-warning',
      };
    case 'Error':
      return {
        bgClass: 'bg-error/15 border-error',
        borderClass: 'border-l-error',
        iconColor: 'text-error',
      };
    default:
      return {
        bgClass: 'bg-base-200 border-base-300',
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
