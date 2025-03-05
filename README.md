# 📚 Lamer Helper
Программа с универстальными утилитами для упрощённого использования персонального компьютера.

| Функциональность | Наличие |
| - | - |
| Изменение цвета выделения курсора мыши | ✅ |
| Создание QR-кода | ✅ |
| Очистка DNS кэша | ✅ |
| Очистка временных файлов Temp | ✅ |
| Очистка кэша обновлений | ✅ |

## ⚙ Документация
### Запуск приложения
1. Скомплируйте проект. На данный момент будет пустое окно с необнаруженными утилитами

![image](https://github.com/user-attachments/assets/b334870f-5917-4a7e-a0a9-8bac6db77e60)

<img width="500" src="https://github.com/user-attachments/assets/d59c26d3-2352-4649-93c5-e46f3d9c96ce">

2. Перейдите в `*папка_проекта*/bin/Release` или `*папка_проекта*/bin/Debug`, и переместите `ModuleConfig.json` к exe-файлу
3. Информация о модулях передана, теперь можете открывать программу с закруженным контентом. Либо компилировать снова
*При изменении информации о модуле не забывайте редактировать и обновлять json-конфиг*

### Создание модуля
1. Перейдите в директорию `Modules` и выберите нужную категорию (Feature, Optimization), в рамках которой будет модуль. В нашем случаи это "Feature"

![image](https://github.com/user-attachments/assets/146b72a8-1904-4d38-a782-7e27795c28c9)

3. Создайте новую папку, например `TestModule`

![image](https://github.com/user-attachments/assets/364538b9-dbfd-4199-905e-42751a3fe500)

4. Перейдите в созданную директорию и добавьте 2 файла: `TestModule.xaml` и `TestModule.xaml.cs`

![image](https://github.com/user-attachments/assets/96a1bc73-f70b-4645-b70b-e727f27e1228)

5. Заполните дефолтным содержанием файлы

**TestModule.xaml**
```xaml
<UserControl x:Class="LamerHelper.Modules.Feature.TestModule"
             xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
             xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
             Height="Auto" Width="Auto">
    <Grid>
        <!-- Your ui components -->
    </Grid>
</UserControl>
```
**TestModule.xaml.cs**
```c#
using System.Windows.Controls;

namespace LamerHelper.Modules.Feature
{
    public partial class TestModule : UserControl, IModule
    {
        public TestModule()
        {
            InitializeComponent();
        }

        public string ModuleName => "TestModule";
        public string DisplayName => "Название модуля";
        public string Category => "Фишка";
        public string Description => "Описание модуля.";
        public UserControl GetModuleControl() => this;

        // Ниже описывается функциональность
    }
}
```
6. Готово! Остаётся внести в конфиг-файл новосозданный модуль
7. Откройте `ModuleConfig.json` и добавьте в него новый массив
```json
{
  "ModuleName": "TestModule",
  "DisplayName": "Название модуля",
  "Category": "Фишка",
  "Type": "LamerHelper.Modules.Feature.TestModule"
}
```
*Не забудьте перенести json-файл в директорию скомпилированного `.exe`*

8. Модуль добавлен

![image](https://github.com/user-attachments/assets/001773e4-1fff-450a-8158-fea79b5f51e0)
