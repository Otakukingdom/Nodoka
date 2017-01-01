//
// Created by mistlight on 12/31/2016.
//

#ifndef NODOKANATIVE_SETTINGSFORM_H
#define NODOKANATIVE_SETTINGSFORM_H

#include <QtWidgets/QWidget>
#include "ui_SettingsForm.h"

namespace Ui {
    class SettingsForm;
}


class SettingsForm : public QWidget  {
public:
    SettingsForm(QWidget *parent = 0);
    virtual ~SettingsForm();

private:
    Ui::SettingsForm *ui;

};


#endif //NODOKANATIVE_SETTINGSFORM_H
