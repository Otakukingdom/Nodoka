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
    <div className="toast toast-top toast-end z-50">
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
    <div className={`alert ${alertClass} shadow-lg`}>
      <Icon className="h-5 w-5" />
      <div className="flex-1">
        {notification.title && (
          <div className="font-medium">{notification.title}</div>
        )}
        <div className="text-sm">{notification.message}</div>
      </div>
      <button
        className="btn btn-ghost btn-sm btn-circle"
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
      return 'alert-success';
    case 'Info':
      return 'alert-info';
    case 'Warning':
      return 'alert-warning';
    case 'Error':
      return 'alert-error';
    default:
      return '';
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
