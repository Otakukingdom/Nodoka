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
  const alertClass = getAlertClass(notification.kind);
  const Icon = getIcon(notification.kind);

  return (
    <div
      className={`alert ${alertClass} shadow-xl border border-base-content/10 toast-slide-in min-w-[300px] max-w-md`}
    >
      <div className="flex items-start gap-3">
        <Icon className="h-5 w-5 shrink-0 mt-0.5" />
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

function getAlertClass(kind: NotificationKind): string {
  switch (kind) {
    case 'Success':
      return 'alert-success bg-success/90';
    case 'Info':
      return 'alert-info bg-info/90';
    case 'Warning':
      return 'alert-warning bg-warning/90';
    case 'Error':
      return 'alert-error bg-error/90';
    default:
      return 'bg-base-200';
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
