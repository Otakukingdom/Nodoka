//
// Created by mistlight on 12/31/2016.
//

#ifndef NODOKANATIVE_SETTINGSFORM_H
#define NODOKANATIVE_SETTINGSFORM_H

#include <QtWidgets/QWidget>
#include <src/model/Directory.h>
#include "ui_SettingsForm.h"

namespace Ui {
    class SettingsForm;
}


class SettingsForm : public QWidget  {
public:
    SettingsForm(Directory* directoryModel, QWidget *parent = 0);
    virtual ~SettingsForm();

private:
    Ui::SettingsForm *ui;
    void setup();

};


#endif //NODOKANATIVE_SETTINGSFORM_H
